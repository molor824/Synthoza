namespace Synthoza;

public class RectangleRenderer(TransformStorage transformStorage) : IStorage<Rectangle>, IRenderHandle
{
    private Dictionary<Entity, Rectangle> _rectangles = [];

    public void Render()
    {
        foreach (var (entity, rectangle) in _rectangles)
        {
            if (!transformStorage.TryGetGlobalTransform(entity, out var globalTransform)) continue;
            rectangle.Render(globalTransform);
        }
    }

    public void AddEntity(Entity entity, Rectangle rectangle)
    {
        _rectangles[entity] = rectangle;
    }

    public void DeleteEntity(Entity entity)
    {
        _rectangles.Remove(entity);
    }
}