using System.Numerics;

namespace Synthoza;

public struct Transform
{
    /// <summary>
    /// Translation of transform
    /// </summary>
    public Vector2 Translation;

    /// <summary>
    /// Rotation and uniform scaling of transform.
    /// The direction of the complex number in polar coordinates indicate the rotation,
    /// while the magnitude indicate uniform scaling. 
    /// </summary>
    public Complex Basis;

    public float Scale => (float)Basis.Magnitude;
    public float Rotation => (float)Basis.Phase;

    public Transform(Vector2 translation, Complex basis)
    {
        Translation = translation;
        Basis = basis;
    }

    public Transform Inverse()
    {
        var basis = Basis;
        var lengthSquared = basis.Real * basis.Real + basis.Imaginary * basis.Imaginary;
        basis = new(basis.Real / lengthSquared, -basis.Imaginary / lengthSquared);
        var translation = -Translation;
        var translationComplex = basis * new Complex(translation.X, translation.Y);
        translation = new((float)translationComplex.Real, (float)translationComplex.Imaginary);
        return new(translation, basis);
    }

    public static Transform FromTranslation(Vector2 translation) => new(translation, Complex.One);
    public static Transform FromBasis(Complex basis) => new(Vector2.Zero, basis);
    public static Transform FromRotation(float radian) => FromBasis(Complex.FromPolarCoordinates(1, radian));
    public static Transform FromScale(float scale) => FromBasis(Complex.FromPolarCoordinates(scale, 0));

    public static readonly Transform Identity = new()
    {
        Translation = Vector2.Zero,
        Basis = Complex.One,
    };

    public static Transform operator *(Complex a, Transform b)
    {
        var basis = a * b.Basis;
        var translationComplex = a * new Complex(b.Translation.X, b.Translation.Y);
        return new(new Vector2((float)translationComplex.Real, (float)translationComplex.Imaginary), basis);
    }
    public static Transform operator *(Transform a, Transform b)
    {
        var transform = a.Basis * b;
        return new(transform.Translation + a.Translation, transform.Basis);
    }

    public static Vector2 operator *(Transform a, Vector2 b)
    {
        var complex = a.Basis * new Complex(b.X, b.Y);
        return new Vector2((float)complex.Real, (float)complex.Imaginary) + a.Translation;
    }
}