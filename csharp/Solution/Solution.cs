using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    // Cantor's Diagonal Argument
    public string FindDifferentBinaryString(string[] nums)
    {
        var n = nums.Length;
        StringBuilder sb = new(n);
        for (int i = 0; i < n; i++)
        {
            sb.Append(nums[i][i] == '1' ? '0' : '1');
        }
        return sb.ToString();
    }
}


