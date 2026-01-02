using System.Diagnostics;

namespace Synthoza;

public class TransformStorage
{
    // Primary data
    private Dictionary<Entity, Transform> _transforms = [];

    // Cache (Eager)
    private Dictionary<Entity, Transform> _globalTransforms = [];

    private HierarchyStorage _hierarchyStorage;

    public TransformStorage(EntityStorage entityStorage, HierarchyStorage storage)
    {
        _hierarchyStorage = storage;
        _hierarchyStorage.Reparented += (entity, _, _) => UpdateGlobalTransforms(entity);

        entityStorage.EntityDeleted += EntityDeleted;
    }

    void EntityDeleted(Entity entity)
    {
        _transforms.Remove(entity);
        _globalTransforms.Remove(entity);
    }

    public bool TryGetParentWithTransform(Entity child, out Entity parent)
    {
        parent = child;
        while (_hierarchyStorage.TryGetParent(parent, out parent))
        {
            if (_transforms.TryGetValue(parent, out _))
                return true;
        }

        parent = default;

        return false;
    }

    // Update global transform of the current entity and every descendant of it
    void UpdateGlobalTransforms(Entity root)
    {
        _globalTransforms[root] = TryGetParentWithTransform(root, out var parent)
            ? _globalTransforms[parent] * _transforms[root]
            : _transforms[root];
        
        foreach (var child in _hierarchyStorage.GetDescendants(root))
        {
            if (!_transforms.ContainsKey(child)) continue;
            if (!TryGetParentWithTransform(child, out parent)) throw new UnreachableException("Descendants cannot parent-less!");
            _globalTransforms[child] = _globalTransforms[parent] * _transforms[child];
        }
    }

    public void SetTransform(Entity entity, Transform transform)
    {
        _transforms[entity] = transform;
        UpdateGlobalTransforms(entity);
    }

    public bool TryGetTransform(Entity entity, out Transform transform) =>
        _transforms.TryGetValue(entity, out transform);

    public Transform GetTransform(Entity entity) => _transforms[entity];

    public bool TryGetGlobalTransform(Entity entity, out Transform globalTransform) =>
        _globalTransforms.TryGetValue(entity, out globalTransform);

    public Transform GetGlobalTransform(Entity entity) => _globalTransforms[entity];
}