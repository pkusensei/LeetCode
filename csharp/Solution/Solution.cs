using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountCollisions(string directions)
    {
        int left = directions.IndexOfAny(['R', 'S']);
        int right = directions.LastIndexOfAny(['L', 'S']);
        if (left == -1 || right == -1 || left > right) { return 0; }
        return directions[left..(1 + right)].Count(c => c != 'S');
    }
}
