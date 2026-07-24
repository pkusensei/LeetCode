using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxRepOpt1(string text)
    {
        Span<int> freq = stackalloc int[26];
        List<(char c, int len)> chunks = [];
        char c = '#';
        int len = 0;
        for (int i = 0; i < text.Length; i++)
        {
            char curr = text[i];
            freq[curr - 'a'] += 1;
            if (curr != c)
            {
                if (len > 0) { chunks.Add((c, len)); }
                c = curr;
                len = 1;
            }
            else
            {
                len += 1;
            }
        }
        chunks.Add((c, len));
        int res = 1;
        int n = chunks.Count;
        for (int i = 0; i < n; i++)
        {
            var curr = chunks[i];
            res = int.Max(res, curr.len);
            if (freq[curr.c - 'a'] > curr.len)
            {
                res = int.Max(res, 1 + curr.len);
            }
            if (0 < i && i < n - 1 && curr.len == 1 && chunks[i - 1].c == chunks[1 + i].c)
            {
                int left = chunks[i - 1].c;
                int val = chunks[i - 1].len + chunks[1 + i].len;
                if (freq[left - 'a'] > val) { res = int.Max(res, 1 + val); }
                else { res = int.Max(res, val); }
            }
        }
        return res;
    }
}
