using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumMatchingSubseq(string s, string[] words)
    {
        Dictionary<char, List<string>> buckets = [];
        foreach (var w in words)
        {
            if (!buckets.TryAdd(w[0], [w[1..]]))
            {
                buckets[w[0]].Add(w[1..]);
            }
        }
        int res = 0;
        foreach (var ch in s)
        {
            if (buckets.Remove(ch, out var lst))
            {
                foreach (var item in lst)
                {
                    if (string.IsNullOrEmpty(item)) { res += 1; }
                    else if (!buckets.TryAdd(item[0], [item[1..]]))
                    {
                        buckets[item[0]].Add(item[1..]);
                    }

                }
            }
        }
        return res;
    }
}

