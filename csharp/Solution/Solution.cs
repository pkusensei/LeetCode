using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;


public class Solution
{
    public IList<int> MajorityElement(int[] nums)
    {
        int num1 = 0;
        int num2 = 0;
        int count1 = 0;
        int count2 = 0;
        foreach (var num in nums)
        {
            if (num == num1) { count1 += 1; }
            else if (num == num2) { count2 += 1; }
            else if (count1 == 0)
            {
                num1 = num;
                count1 = 1;
            }
            else if (count2 == 0)
            {
                num2 = num;
                count2 = 1;
            }
            else
            {
                count1 -= 1;
                count2 -= 1;
            }
        }
        count1 = nums.Count(v => v == num1);
        count2 = nums.Count(v => v == num2);
        List<int> res = [];
        if (count1 > nums.Length / 3) { res.Add(num1); }
        if (count2 > nums.Length / 3 && num2 != num1) { res.Add(num2); }
        return res;
    }
}