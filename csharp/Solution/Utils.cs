using System.Numerics;
using System.Text;

namespace Solution;

public static class Utils
{
    public static string Print<T>(this IEnumerable<T> values)
    {
        var sb = new StringBuilder();
        sb.Append('[');
        sb.AppendJoin(',', values);
        sb.Append(']');
        return sb.ToString();
    }

    public static T ModPow<T>(T b, T exp, T mod) where T : IBinaryInteger<T>
    {
        if (exp == T.Zero) { return T.One; }
        if ((exp & T.One) == T.Zero) { return ModPow(b * b % mod, exp >> 1, mod); }
        else { return ModPow(b * b % mod, exp >> 1, mod) * b % mod; }
    }
}