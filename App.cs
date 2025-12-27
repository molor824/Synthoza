using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class App
{
    private List<ILoopHandle> _loopHandles = [];

    public App(string title, int width, int height)
    {
        Raylib.SetConfigFlags(ConfigFlags.MaximizedWindow | ConfigFlags.VSyncHint | ConfigFlags.AlwaysRunWindow);
        Raylib.InitWindow(width, height, title);
    }

    public void AddHandles(params ILoopHandle[] handles) => _loopHandles.AddRange(handles);

    // Screen size is measured in actual pixels of the window size
    public int ScreenWidth => Raylib.GetScreenWidth();
    public int ScreenHeight => Raylib.GetScreenHeight();
    public Vector2 ScreenSize => new(ScreenWidth, ScreenHeight);

    // Render size is measured in size of the render frame itself, which depending on window configuration
    // may differ from screen size
    public int RenderWidth => Raylib.GetRenderWidth();
    public int RenderHeight => Raylib.GetRenderHeight();
    public Vector2 RenderSize => new(RenderWidth, RenderHeight);

    // Resizes in screen size
    public void Resize(int width, int height)
    {
        Raylib.SetWindowSize(width, height);
    }

    public void Run()
    {
        while (!Raylib.WindowShouldClose())
        {
            var delta = Raylib.GetFrameTime();
            foreach (var handle in _loopHandles)
                handle.Update(delta);
        }
    }
}

public interface ILoopHandle
{
    void Update(float deltaTime);
}