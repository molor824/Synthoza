using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class Rectangle
{
    public Transform Transform = Transform.Identity;
    public Color Color = Color.White;
    public Color? BorderColor;
    public float BorderWidth;
    public Vector2 Size;
    public Vector2 Anchor = new(0.5f, 0.5f);

    public Rectangle(Vector2 size)
    {
        Size = size;
    }

    public bool Intersecting(Transform globalTransform, Vector2 point)
    {
        var inverse = (globalTransform * Transform).Inverse();
        var localPoint = inverse * point;
        var start = -Size * Anchor;
        var end = Size * (new Vector2(1) - Anchor);
        return localPoint.X >= start.X && localPoint.X <= end.X && localPoint.Y >= start.Y && localPoint.Y <= end.Y;
    }

    public void Render(Transform globalTransform)
    {
        var transform = globalTransform * Transform;
        var translation = transform.Translation;
        var rotation = transform.Rotation * 180 / MathF.PI;
        var scale = transform.Scale;
        var size = Size * scale;
        if (BorderColor is not null)
        {
            var borderSize = size;
            var borderTranslation = translation;
            var offset = (new Vector2(0.5f) - Anchor) * new Vector2(BorderWidth * 2);
            translation = transform * offset;
            size = (Size - new Vector2(BorderWidth * 2)) * scale;
            Raylib.DrawRectanglePro(new(borderTranslation, borderSize), borderSize * Anchor, rotation,
                BorderColor.Value);
        }

        Raylib.DrawRectanglePro(new(translation, size), size * Anchor, rotation, Color);
    }
}