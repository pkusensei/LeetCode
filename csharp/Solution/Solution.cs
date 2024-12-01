using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool CheckIfExist(int[] arr)
    {
        var seen = new HashSet<int>();
        foreach (var num in arr)
        {
            if (seen.Contains(2 * num)) { return true; }
            if ((num & 1) == 0 && seen.Contains(num / 2)) { return true; }
            seen.Add(num);
        }
        return false;
    }
}
