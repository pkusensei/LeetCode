using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string LargestNumber(int[] nums)
    {
        string[] strs = [.. nums.Select(v => v.ToString())];
        Array.Sort(strs, (a, b) => (b + a).CompareTo(a + b));
        if (strs[0] == "0") { return "0"; }
        return string.Join("", strs);
    }
}
