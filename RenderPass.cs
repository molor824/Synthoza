using Raylib_cs;

namespace Synthoza;

public class RenderPass : ILoopHandle
{
    private List<IRenderHandle> _renderHandles = [];

    public Color ClearColor = Color.Black;

    public void AddHandles(params IRenderHandle[] handles) => _renderHandles.AddRange(handles);

    public void Update(float deltaTime)
    {
        Raylib.BeginDrawing();
        
        Raylib.ClearBackground(ClearColor);

        foreach (var handle in _renderHandles)
            handle.Render();
        
        Raylib.EndDrawing();
    }
}

public interface IRenderHandle
{
    void Render();
}