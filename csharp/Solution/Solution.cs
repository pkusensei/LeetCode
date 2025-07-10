using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    static int[] F => [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    public string GetPermutation(int n, int k)
    {
        List<char> cands = [.. Enumerable.Range(0, n).Select(i => (char)(i + '1'))];
        List<char> res = [];
        Dfs(res, cands, k);
        return string.Concat(res);

        static void Dfs(List<char> res, List<char> cands, int k)
        {
            if (cands.Count == 0) { return; }
            int subtree = F[cands.Count - 1]; // e.g n=9, subtree=(8!)
            int idx = 0;
            while (k > subtree)
            {
                k -= subtree;
                idx += 1;
            }
            res.Add(cands[idx]);
            cands.RemoveAt(idx);
            Dfs(res, cands, k);
        }
    }
}