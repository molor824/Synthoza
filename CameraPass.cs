using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class CameraPass : IRenderHandle
{
    public Transform Transform = Transform.Identity;
    public Camera2D Camera2D => new()
    {
        Offset = _app.RenderSize * 0.5f,
        Rotation = (float)(Transform.Basis.Phase * 180.0 / Math.PI),
        Target = Transform.Translation,
        Zoom = (float)Transform.Basis.Magnitude
    };

    private List<ICameraHandle> _cameraHandles = [];
    private App _app;

    public CameraPass(App app)
    {
        _app = app;
    }

    public Vector2 ScreenToWorld(Vector2 screen)
    {
        screen -= _app.RenderSize * 0.5f;
        return Transform * screen;
    }

    public void AddHandle(ICameraHandle handle) => _cameraHandles.Add(handle);

    public void Render()
    {
        Raylib.BeginMode2D(Camera2D);

        foreach (var handle in _cameraHandles)
            handle.CameraRender();

        Raylib.EndMode2D();
    }
}

public interface ICameraHandle
{
    void CameraRender();
}