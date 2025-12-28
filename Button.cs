using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class Button : Rectangle
{
    public Color NormalColor = Color.LightGray;
    public Color ClickColor = Color.Gray;
    public Color HoverColor = Color.White;
    public bool Hovering => _hovering;
    public bool Clicking => _clicking;

    private bool _hovering, _clicking;

    public Button(Vector2 size) : base(size)
    {
    }

    public void Update(Transform globalTransform)
    {
        var mousePosition = Raylib.GetMousePosition();
        
        _hovering = Intersecting(globalTransform, mousePosition);
        if (_hovering && Raylib.IsMouseButtonPressed(MouseButton.Left))
            _clicking = true;
        if (Raylib.IsMouseButtonUp(MouseButton.Left))
            _clicking = false;

        if (_clicking)
            Color = ClickColor;
        else if (_hovering)
            Color = HoverColor;
        else
            Color = NormalColor;
    }
}
