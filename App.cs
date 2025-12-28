using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class App
{
    private List<ILoopHandle> _loopHandles = [];

    public App(string title, int width, int height)
    {
        Raylib.SetConfigFlags(ConfigFlags.MaximizedWindow | ConfigFlags.VSyncHint | ConfigFlags.AlwaysRunWindow |
                              ConfigFlags.ResizableWindow);
        Raylib.InitWindow(width, height, title);
        Raylib.InitAudioDevice();
    }

    public void AddHandles(params ILoopHandle[] handles) => _loopHandles.AddRange(handles);

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