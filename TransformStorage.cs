namespace Synthoza;

public class TransformStorage : IStorage<Transform>
{
    // Primary data
    private Dictionary<Entity, Transform> _transforms = [];

    // Cache (Lazy)
    private Dictionary<Entity, Transform> _globalTransforms = [];

    private HierarchyStorage _hierarchyStorage;

    public TransformStorage(HierarchyStorage storage)
    {
        _hierarchyStorage = storage;
        _hierarchyStorage.Reparented += (entity, _, _) => DeleteGlobalTransforms(entity);
    }

    public void DeleteEntity(Entity entity)
    {
        _transforms.Remove(entity);
        _globalTransforms.Remove(entity);
    }

    public void AddEntity(Entity entity, Transform transform)
    {
        _transforms[entity] = transform;
        DeleteGlobalTransforms(entity);
    }

    bool TryGetParentWithTransform(Entity child, out Entity parent, out Transform parentTransform)
    {
        parent = child;
        while (_hierarchyStorage.TryGetParent(parent, out parent))
        {
            if (_transforms.TryGetValue(parent, out parentTransform))
                return true;
        }

        parent = default;
        parentTransform = default;

        return false;
    }
    
    // Delete global transforms propagating down the tree
    void DeleteGlobalTransforms(Entity root)
    {
        _globalTransforms.Remove(root);
        foreach (var child in _hierarchyStorage.GetDescendants(root))
            _globalTransforms.Remove(child);
    }

    // Calculate and store every global transforms from entity up to the parent with global transform
    // WARNING: Assumes entity has local transform!
    Transform CalculateGlobalTransforms(Entity entity)
    {
        // Check if entity has cached global transform, if so return immediately
        if (_globalTransforms.TryGetValue(entity, out var globalTransform))
            return globalTransform;
        
        // Global transform start with identity so that when there is no parent,
        // multiplying by parent's transform results in itself (as expected)
        globalTransform = Transform.Identity;
        // Stack storing both parent entities and local transforms
        // Include the entity itself, so that the actual entity's transform to return gets calculated aswell
        var parentStack = new Stack<(Entity, Transform)>();
        parentStack.Push((entity, _transforms[entity]));
        var parent = entity;

        // Iterate through parents
        while (TryGetParentWithTransform(parent, out parent, out var parentTransform))
        {
            if (_globalTransforms.TryGetValue(parent, out globalTransform))
                break; // Found a parent with global transform, no need to iterate further
            // Push parent and it's local transform
            parentStack.Push((parent, parentTransform));
        }
        
        // Propagate back
        while (parentStack.TryPop(out var entityTransform))
        {
            (parent, var transform) = entityTransform;
            globalTransform *= transform; // Update global transform such that local transform should come first
            _globalTransforms[parent] = globalTransform;
        }

        return globalTransform;
    }

    public void SetTransform(Entity entity, Transform transform)
    {
        _transforms[entity] = transform;
        DeleteGlobalTransforms(entity);
    }

    public bool TryGetTransform(Entity entity, out Transform transform) =>
        _transforms.TryGetValue(entity, out transform);

    public Transform GetTransform(Entity entity) => _transforms[entity];

    public bool TryGetGlobalTransform(Entity entity, out Transform globalTransform)
    {
        if (!_transforms.ContainsKey(entity))
        {
            globalTransform = default;
            return false;
        }

        globalTransform = CalculateGlobalTransforms(entity);
        return true;
    }

    public Transform GetGlobalTransform(Entity entity) => CalculateGlobalTransforms(entity);
}