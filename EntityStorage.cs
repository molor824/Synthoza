namespace Synthoza;

public class EntityStorage
{
    public long Counter { get; private set; } = 1;
    
    public delegate void EntityDeletedCallback(Entity entity);

    public event EntityDeletedCallback? EntityDeleted;

    private HashSet<Entity> _entities = [];

    public Entity CreateNew()
    {
        var entity = new Entity(Counter++);
        _entities.Add(entity);
        return entity;
    }
    
    public bool ContainsEntity(Entity entity) => _entities.Contains(entity);

    public bool DeleteEntity(Entity entity)
    {
        if (_entities.Remove(entity))
        {
            EntityDeleted?.Invoke(entity);
            return true;
        }

        return false;
    }
}

public record struct Entity(long Id);