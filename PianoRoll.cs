using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class PianoRoll : IRenderHandle, ILoopHandle
{
    public const int Octaves = 2;
    public const float KeyDistances = 20;
    public const float WhiteKeyLength = 200;
    public const float BlackKeyLength = 120;

    public static readonly Color WhiteKeyColor = Color.White;
    public static readonly Color WhiteKeyHoverColor = new(210, 210, 210, 255);
    public static readonly Color WhiteKeyClickColor = new(180, 180, 180, 255);

    public static readonly Color BlackKeyColor = Color.DarkGray;
    public static readonly Color BlackKeyHoverColor = new(40, 40, 40, 255);
    public static readonly Color BlackKeyClickColor = new(0, 0, 0, 255);

    public static readonly bool[] WhiteKeys =
        [true, false, true, false, true, true, false, true, false, true, false, true];

    public static readonly Color[] KeyColors =
    [
        WhiteKeyColor,
        BlackKeyColor,
        WhiteKeyColor,
        BlackKeyColor,
        WhiteKeyColor,
        WhiteKeyColor,
        BlackKeyColor,
        WhiteKeyColor,
        BlackKeyColor,
        WhiteKeyColor,
        BlackKeyColor,
        WhiteKeyColor,
    ];

    public static readonly Vector2[] KeySizes =
    [
        new(WhiteKeyLength, KeyDistances * 1.5f),
        new(BlackKeyLength, KeyDistances),
        new(WhiteKeyLength, KeyDistances * 2),
        new(BlackKeyLength, KeyDistances),
        new(WhiteKeyLength, KeyDistances * 1.5f),
        new(WhiteKeyLength, KeyDistances * 1.5f),
        new(BlackKeyLength, KeyDistances),
        new(WhiteKeyLength, KeyDistances * 2),
        new(BlackKeyLength, KeyDistances),
        new(WhiteKeyLength, KeyDistances * 2),
        new(BlackKeyLength, KeyDistances),
        new(WhiteKeyLength, KeyDistances * 1.5f),
    ];

    public static readonly Vector2[] KeyAnchors =
    [
        new(0, 2f / 3f),
        new(0, 0.5f),
        new(0, 0.5f),
        new(0, 0.5f),
        new(0, 1f / 3f),
        new(0, 2f / 3f),
        new(0, 0.5f),
        new(0, 0.5f),
        new(0, 0.5f),
        new(0, 0.5f),
        new(0, 0.5f),
        new(0, 1f / 3f),
    ];

    private Transform _transform = Transform.Identity;
    private Rectangle[] _pianoKeys;

    public PianoRoll()
    {
        _pianoKeys = new Rectangle[Octaves * 12];
        var totalWidth = Octaves * 12 * KeyDistances;
        for (int i = 0; i < _pianoKeys.Length; i++)
        {
            var note = i % 12;
            _pianoKeys[i] = new Rectangle(KeySizes[note])
            {
                Anchor = KeyAnchors[note],
                Color = KeyColors[note],
                BorderWidth = 2,
                BorderColor = Color.Black,
            };
        }
    }

    public void Update(float deltaTime)
    {
        int? hoveringKey = null;
        var mousePosition = Raylib.GetMousePosition();
        for (int i = 0; i < _pianoKeys.Length && hoveringKey is null; i++)
        {
            var note = i % 12;
            if (!WhiteKeys[note] && _pianoKeys[i].Intersecting(_transform, mousePosition))
                hoveringKey = i;
        }

        for (int i = 0; i < _pianoKeys.Length && hoveringKey is null; i++)
        {
            var note = i % 12;
            if (WhiteKeys[note] && _pianoKeys[i].Intersecting(_transform, mousePosition))
                hoveringKey = i;
        }

        for (int i = 0; i < _pianoKeys.Length; i++)
        {
            var key = _pianoKeys[i];
            var note = i % 12;
            if (i == hoveringKey)
            {
                if (Raylib.IsMouseButtonDown(MouseButton.Left))
                    key.Color = WhiteKeys[note] ? WhiteKeyClickColor : BlackKeyClickColor;
                else
                    key.Color = WhiteKeys[note] ? WhiteKeyHoverColor : BlackKeyHoverColor;
            }
            else
            {
                key.Color = WhiteKeys[note] ? WhiteKeyColor : BlackKeyColor;
            }
        }
    }

    public void Render()
    {
        for (int i = 0; i < _pianoKeys.Length; i++)
        {
            var note = i % 12;
            if (WhiteKeys[note])
                _pianoKeys[i].Render(_transform);
        }

        for (int i = 0; i < _pianoKeys.Length; i++)
        {
            var note = i % 12;
            if (!WhiteKeys[note])
                _pianoKeys[i].Render(_transform);
        }
    }
}