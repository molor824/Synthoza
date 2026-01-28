using System.Numerics;
using Raylib_cs;

namespace Synthoza.Gui;

public class UiRectStorage(HierarchyStorage hierarchyStorage, TransformStorage transformStorage) : IStorage<UiRect>, ILoopHandle
{
    private Dictionary<Entity, UiRect> _uiRects = [];
    private TransformStorage _transformStorage = transformStorage;
    private HierarchyStorage _hierarchyStorage = hierarchyStorage;
    
    public void AddEntity(Entity entity, UiRect data)
    {
        _uiRects[entity] = data;
    }

    public void DeleteEntity(Entity entity)
    {
        _uiRects.Remove(entity);
    }

    void SyncTransform(Dictionary<Entity, Transform> syncedEntities, Entity entity)
    {
        
    }

    public void Update(float deltaTime)
    {
        if (!Raylib.IsWindowResized())
            return;

        var syncedEntities = new Dictionary<Entity, Transform>(_uiRects.Count);

        foreach (var entity in _uiRects.Keys)
        {
            if (syncedEntities.ContainsKey(entity)) continue;
            SyncTransform(syncedEntities, entity);
        }
    }
}