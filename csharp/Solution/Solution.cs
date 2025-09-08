using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LeastBricks(IList<IList<int>> wall)
    {
        Dictionary<int, int> freq = [];
        int curr = 0;
        foreach (var row in wall)
        {
            curr = 0;
            foreach (var width in row)
            {
                curr += width;
                if (!freq.TryAdd(curr, 1)) { freq[curr] += 1; }
            }
        }
        freq.Remove(curr);
        if (freq.Count == 0) { return wall.Count; }
        return wall.Count - freq.Values.Max();
    }
}
