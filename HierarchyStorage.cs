using System.Collections.Immutable;

namespace Synthoza;

public class HierarchyStorage : IStorage<Entity>
{
    // Primary data
    private Dictionary<Entity, Entity> _parentMap = [];
    // Cache (Eager)
    private Dictionary<Entity, HashSet<Entity>> _childrenMap = [];

    public delegate void ReparentedCallback(Entity entity, Entity? newParent, Entity? oldParent);

    public event ReparentedCallback? Reparented;

    public void DeleteEntity(Entity entity)
    {
        // Reparent the current entity to gracefully remove entity
        Reparent(entity, null);
        // Reparent each child
        if (_childrenMap.TryGetValue(entity, out var children))
        {
            foreach (var child in children)
                Reparent(child, null);
        }
    }

    public void AddEntity(Entity entity, Entity parent)
    {
        Reparent(entity, parent);
    }

    public IReadOnlySet<Entity> GetChildren(Entity entity)
    {
        if (_childrenMap.TryGetValue(entity, out var children)) return children;
        return ImmutableHashSet<Entity>.Empty;
    }

    public IEnumerable<Entity> GetDescendants(Entity entity)
    {
        var stack = new Stack<Entity>(GetChildren(entity));

        while (stack.TryPop(out var child))
        {
            yield return child;
            foreach (var child1 in GetChildren(child))
                stack.Push(child1);
        }
    }

    public Entity GetParent(Entity child)
    {
        if (!TryGetParent(child, out var parent))
            throw new ParentNotFoundException(child);
        return parent;
    }
    public bool TryGetParent(Entity entity, out Entity parent) => _parentMap.TryGetValue(entity, out parent);

    public bool Reparent(Entity entity, Entity? newParent, out Entity? oldParent)
    {
        // Try to remove old parent
        oldParent = _parentMap.GetValueOrDefault(entity);
        
        // If newparent and oldparent has same value then no point trying to reparent
        if (oldParent == newParent) return false;
        
        // If oldParent exists, remove entity from children set
        if (oldParent.HasValue)
        {
            if (_childrenMap.TryGetValue(oldParent.Value, out var oldChildren))
                oldChildren.Remove(entity);
        }
        
        // If new parent exists, switch to newParent and add entity to children set
        if (newParent.HasValue)
        {
            var newParent1 = newParent.Value;
            _parentMap[entity] = newParent1;
            if (_childrenMap.TryGetValue(newParent1, out var children))
                children.Add(entity);
            else
                _childrenMap[newParent1] = [entity];
        }
        // Otherwise remove parent
        else
        {
            _parentMap.Remove(entity);
        }

        // Reparenting was succesful, call the event
        Reparented?.Invoke(entity, newParent, oldParent);
        return true;
    }

    public bool Reparent(Entity entity, Entity? newParent) => Reparent(entity, newParent, out _);
}

public class ParentNotFoundException(Entity child) : KeyNotFoundException($"No parent of {child} found")
{
}