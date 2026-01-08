using Raylib_cs;

namespace Synthoza;

public class App
{
    private List<ILoopHandle> _loopHandles = [];
    private List<IDisposable> _disposables = [];

    public App(string title, int width, int height)
    {
        Raylib.SetConfigFlags(ConfigFlags.MaximizedWindow | ConfigFlags.VSyncHint | ConfigFlags.AlwaysRunWindow |
                              ConfigFlags.ResizableWindow | ConfigFlags.Msaa4xHint);
        Raylib.InitWindow(width, height, title);
        Raylib.InitAudioDevice();
    }

    public void AddHandles(params ILoopHandle[] handles) => _loopHandles.AddRange(handles);
    public void AddDisposables(params IDisposable[] disposables) => _disposables.AddRange(disposables);

    public void Run()
    {
        while (!Raylib.WindowShouldClose())
        {
            var delta = Raylib.GetFrameTime();
            foreach (var handle in _loopHandles)
                handle.Update(delta);
        }
        
        foreach (var disposable in _disposables)
            disposable.Dispose();
        
        Raylib.CloseWindow();
        Raylib.CloseAudioDevice();
    }
}

public interface ILoopHandle
{
    void Update(float deltaTime);
}
