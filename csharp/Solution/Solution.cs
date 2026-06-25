using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumTilePossibilities(string tiles)
    {
        HashSet<string> set = [];
        StringBuilder sb = new();
        Backtrack(0);
        return set.Count - 1;

        void Backtrack(int idx)
        {
            if (idx >= tiles.Length)
            {
                set.Add(sb.ToString());
                return;
            }
            Backtrack(1 + idx);
            int n = sb.Length;
            for (int i = 0; i <= n; i++)
            {
                sb.Insert(i, tiles[idx]);
                Backtrack(1 + idx);
                sb.Remove(i, 1);
            }
        }
    }
}
