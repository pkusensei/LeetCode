using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxDifference(string s, int k)
    {
        const int INF = int.MaxValue / 2;
        int res = int.MinValue;
        foreach (var a in "01234")
        {
            foreach (var b in "01234")
            {
                if (a == b) { continue; }
                List<int> prefa = [0];
                List<int> prefb = [0];
                Dictionary<(int, int), int> seen = [];
                int left = 0;
                for (int idx = 0; idx < s.Length; idx += 1)
                {
                    prefa.Add(prefa.Last());
                    prefb.Add(prefb.Last());
                    if (s[idx] == a) { prefa[1 + idx] += 1; }
                    if (s[idx] == b) { prefb[1 + idx] += 1; }
                    while (left + k - 1 <= idx && prefa[left] < prefa[1 + idx] && prefb[left] < prefb[1 + idx])
                    {
                        var key = (prefa[left] & 1, prefb[left] & 1);
                        int diff = prefa[left] - prefb[left];
                        if (!seen.TryAdd(key, diff)) { seen[key] = Math.Min(seen[key], diff); }
                        left += 1;
                    }
                    var key_ = (1 - (prefa[1 + idx] & 1), prefb[1 + idx] & 1);
                    int diff_ = prefa[1 + idx] - prefb[1 + idx];
                    int prev = seen.GetValueOrDefault(key_, INF);
                    res = Math.Max(res, diff_ - prev);
                }
            }
        }
        return res;
    }
}
