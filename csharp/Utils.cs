using System.Text;

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
}