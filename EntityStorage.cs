namespace Synthoza;

public class EntityStorage
{
    public long Counter => _counter;

    private long _counter;
    private HashSet<Entity> _entities = [];
    private Dictionary<Type, IStorage> _componentStorages = [];

    public Entity CreateNew(params object[] components)
    {
        var entity = new Entity(Interlocked.Increment(ref _counter));
        _entities.Add(entity);
        foreach (var component in components)
            _componentStorages[component.GetType()].AddEntity(entity, component);
        return entity;
    }

    public void AddStorage(IStorage storage) => _componentStorages[storage.ComponentType] = storage;

    public void AddStorages(params IStorage[] storages)
    {
        foreach (var storage in storages)
            AddStorage(storage);
    }
    public bool ContainsEntity(Entity entity) => _entities.Contains(entity);

    public bool DeleteEntity(Entity entity)
    {
        if (_entities.Remove(entity))
        {
            foreach (var storage in _componentStorages.Values)
                storage.DeleteEntity(entity);
            return true;
        }

        return false;
    }
}

public interface IStorage
{
    Type ComponentType { get; }
    void DeleteEntity(Entity entity);
    void AddEntity(Entity entity, object data);
}

public interface IStorage<in T> : IStorage
{
    Type IStorage.ComponentType => typeof(T);
    void AddEntity(Entity entity, T data);
    void IStorage.AddEntity(Entity entity, object data)
    {
        AddEntity(entity, (T)data);
    }
}

public record struct Entity(long Id);