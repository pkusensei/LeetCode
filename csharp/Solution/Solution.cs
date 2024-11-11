using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int RegionsBySlashes(string[] grid)
    {
        var n = grid.Length;
        var side = 1 + n;
        var total = side * side;
        int[] parents = new int[total];
        Array.Fill(parents, -1);
        for (int r = 0; r < side; r++)
        {
            for (int c = 0; c < side; c++)
            {
                if (r == 0 || c == 0 || r == side - 1 || c == side - 1)
                {
                    var p = r * side + c;
                    parents[p] = 0;
                }
            }
        }
        parents[0] = -1;
        var res = 1;
        foreach (var (r, row) in grid.Select((r, i) => (i, r)))
        {
            foreach (var (c, ch) in row.Select((c, i) => (i, c)))
            {
                if (ch == '/')
                {
                    var top_right = r * side + c + 1;
                    var bottom_left = (r + 1) * side + c;
                    res += Union(parents, top_right, bottom_left);
                }
                else if (ch == '\\')
                {
                    var top_left = r * side + c;
                    var botttom_right = (r + 1) * side + c + 1;
                    res += Union(parents, top_left, botttom_right);
                }
            }
        }
        return res;
    }

    static int Find(int[] parents, int p)
    {
        if (parents[p] == -1) { return p; }
        parents[p] = Find(parents, parents[p]);
        return parents[p];
    }

    static int Union(int[] parents, int x, int y)
    {
        var px = Find(parents, x);
        var py = Find(parents, y);
        if (px == py) { return 1; } // A loop
        parents[py] = px;
        return 0;
    }
}
