namespace Synthoza;

internal static class Program
{
    private static void Main(string[] args)
    {
        var app = new App("Synthoza", 1280, 720);

        var entityStorage = new EntityStorage();
        var hierarchyStorage = new HierarchyStorage();
        var transformStorage = new TransformStorage(hierarchyStorage);
        var rectangleRenderer = new RectangleRenderer(transformStorage);

        entityStorage.AddStorages(hierarchyStorage, transformStorage, rectangleRenderer);
        
        var transformTest = new TransformTest(entityStorage, transformStorage);
        var renderPass = new RenderPass();
        
        renderPass.AddHandles(rectangleRenderer);
        app.AddHandles(transformTest, renderPass);

        app.Run();
    }
}