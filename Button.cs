using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class Button : Rectangle
{
    public Color NormalColor = Color.LightGray;
    public Color ClickColor = Color.Gray;
    public Color HoverColor = Color.White;

    private CameraPass _cameraPass;

    public Button(CameraPass cameraPass, Vector2 size) : base(size)
    {
        _cameraPass = cameraPass;
    }

    public void Update(Transform globalTransform)
    {
        var mousePosition = _cameraPass.ScreenToWorld(Raylib.GetMousePosition());
        if (Intersecting(globalTransform, mousePosition))
        {
            if (Raylib.IsMouseButtonDown(MouseButton.Left))
                Color = ClickColor;
            else
                Color = HoverColor;
        }
        else
        {
            Color = NormalColor;
        }
    }
}
