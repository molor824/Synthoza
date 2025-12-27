using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class PianoRoll : ICameraHandle
{
    public const int Octaves = 2;
    public const float KeyDistances = 40;
    public const float WhiteKeyLength = 200;
    public const float BlackKeyLength = 120;
    public static readonly Color WhiteKeyColor = Color.White;
    public static readonly Color BlackKeyColor = Color.DarkGray;
    public static readonly int[] WhiteKeyIndices = [0, 2, 4, 5, 7, 9, 11];
    public static readonly int[] BlackKeyIndices = [1, 3, 6, 8, 10];

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

    private Transform _transform = Transform.FromRotation(MathF.PI / 2f);
    private Button[] _pianoKeys;
    private CameraPass _cameraPass;

    public PianoRoll(CameraPass cameraPass)
    {
        _pianoKeys = new Button[Octaves * 12];
        _cameraPass = cameraPass;
        var totalWidth = Octaves * 12 * KeyDistances;
        var center = totalWidth * 0.5f;
        for (int i = 0; i < _pianoKeys.Length; i++)
        {
            var note = i % 12;
            _pianoKeys[i] = new Button(cameraPass, KeySizes[note])
            {
                Anchor = KeyAnchors[note],
                Transform = Transform.FromTranslation(new(0, center - i * KeyDistances)),
                Color = KeyColors[note],
                NormalColor = KeyColors[note],
                HoverColor = Color.Gray,
                ClickColor = KeyColors[note],
                BorderWidth = 2,
                BorderColor = Color.Black,
            };
        }
    }

    public void CameraRender()
    {
        foreach (var key in _pianoKeys)
            key.Update(_transform);
            
        for (int octave = 0; octave < Octaves; octave++)
        {
            foreach (var whiteIndex in WhiteKeyIndices)
                _pianoKeys[octave * 12 + whiteIndex].Render(_transform);
            foreach (var blackIndex in BlackKeyIndices)
                _pianoKeys[octave * 12 + blackIndex].Render(_transform);
        }
    }
}