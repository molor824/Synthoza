using System.Numerics;

namespace Synthoza.Gui;

/// <summary>
/// Ui Rectangle that has parent-relative anchors with pixel offsets.
/// It is not recommended to use Transform directly when UiRect is included as a component.
/// Transform is updated everytime UiRect is updated, or when window resizes.
/// </summary>
public record struct UiRect
{
    /// <summary>
    /// Start/End Anchors.
    /// Determines rectangle's start and end coordinates by interpolating the anchors by the parent's start and end coordinates.
    /// </summary>
    public Vector2 StartAnchor, EndAnchor;
    /// <summary>
    /// Start/End Offsets. Adds pixel offsets on top of anchor coordinates.
    /// </summary>
    public Vector2 StartOffset, EndOffset;
}