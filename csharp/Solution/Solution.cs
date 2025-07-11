using System.Text;
using System.Text.RegularExpressions;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> FullJustify(string[] words, int maxWidth)
    {
        List<Line> lines = [];
        Line curr = new(maxWidth);
        foreach (var item in words)
        {
            if (curr.Empty || maxWidth - curr.SingleSpaceLen > item.Length)
            {
                curr.Add(item);
            }
            else
            {
                lines.Add(curr);
                curr = new(maxWidth);
                curr.Add(item);
            }
        }
        curr.IsLast = true;
        lines.Add(curr);
        return [.. lines.Select(v => v.ToString())];
    }
}

internal class Line
{
    public int MaxWidth { get; }
    public List<string> Words { get; }
    public bool IsLast { get; set; }

    public bool Empty => Words.Count == 0;
    public int SingleSpaceLen => Words.Select(w => w.Length + 1).Sum() - 1;

    public Line(int maxWidth, bool isLast = false)
    {
        MaxWidth = maxWidth;
        Words = [];
        IsLast = isLast;
    }

    public void Add(string s) => Words.Add(s);

    public override string ToString()
    {
        StringBuilder sb = new();
        if (Words.Count == 1)
        {
            sb.Append(Words[0]);
        }
        else if (IsLast)
        {
            foreach (var item in Words)
            {
                sb.Append($"{item} ");
            }
        }
        else
        {
            int space = MaxWidth - Words.Select(w => w.Length).Sum();
            int count = Words.Count - 1;
            int ave = space / count;
            int rem = space % count;
            foreach (var item in Words)
            {
                sb.Append(item);
                sb.Append(' ', ave);
                if (rem > 0)
                {
                    sb.Append(' ');
                    rem -= 1;
                }
            }
        }
        while (sb.Length > MaxWidth) { sb.Remove(sb.Length - 1, 1); }
        while (sb.Length < MaxWidth) { sb.Append(' '); }
        return sb.ToString();
    }
}