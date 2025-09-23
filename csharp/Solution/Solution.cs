using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string PredictPartyVictory(string senate)
    {
        int n = senate.Length;
        Queue<int> rs = [];
        Queue<int> ds = [];
        for (int i = 0; i < n; i++)
        {
            if (senate[i] == 'R') { rs.Enqueue(i); }
            else { ds.Enqueue(i); }
        }
        while (ds.Count > 0 && rs.Count > 0)
        {
            int r = rs.Dequeue();
            int d = ds.Dequeue();
            if (r < d) { rs.Enqueue(r + n); }
            else { ds.Enqueue(d + n); }
        }
        if (rs.Count > 0) { return "Radiant"; }
        return "Dire";
    }
}