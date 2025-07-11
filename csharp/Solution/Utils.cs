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

    public static int BitCount<T>(T value) where T : IBinaryInteger<T>
    {
        int res = 0;
        while (value != T.Zero)
        {
            res += 1;
            value &= value - T.One;
        }
        return res;
    }

    public static T ModPow<T>(T b, T exp, T mod) where T : IBinaryInteger<T>
    {
        if (exp == T.Zero) { return T.One; }
        if ((exp & T.One) == T.Zero) { return ModPow(b * b % mod, exp >> 1, mod); }
        else { return ModPow(b * b % mod, exp >> 1, mod) * b % mod; }
    }

    public static T NChooseK<T>(T n, T k, T mod) where T : IBinaryInteger<T>
    {
        if (k > n) { return T.Zero; }
        if (n - k < k) { k = n - k; }
        T numerator = T.One;
        for (T i = n - k + T.One; i <= n; i++)
        {
            numerator = numerator * i % mod;
        }
        T denominator = T.One;
        for (T i = T.One; i <= k; i++)
        {
            denominator = denominator * i % mod;
        }
        return numerator * ModPow(denominator, mod - T.One - T.One, mod);
    }
}