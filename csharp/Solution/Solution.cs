using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int ShoppingOffers(IList<int> price, IList<IList<int>> special, IList<int> needs)
    {
        int n = needs.Count;
        Dictionary<int, int> memo = [];
        special = [.. special.OrderDescending(Comparer<IList<int>>.Create((a, b) => a[n].CompareTo(b[n])))];
        return Dfs(0);

        int Dfs(int idx)
        {
            int bits = 0;
            foreach (var v in needs)
            {
                if (v < 0) { return int.MaxValue / 2; }
                bits = (bits << 4) | v;
            }
            if (memo.TryGetValue(bits, out var seen)) { return seen; }
            if (idx >= special.Count)
            {
                return price.Zip(needs).Sum(p => p.First * p.Second);
            }
            int skip = Dfs(1 + idx); // skip
            for (int i = 0; i < needs.Count; i++)
            {
                needs[i] -= special[idx][i];
            }
            int take = special[idx][^1] + int.Min(Dfs(idx), Dfs(1 + idx));
            for (int i = 0; i < needs.Count; i++)
            {
                needs[i] += special[idx][i]; // backtrack
            }
            int res = int.Min(skip, take);
            memo[bits] = res;
            return res;
        }
    }
}