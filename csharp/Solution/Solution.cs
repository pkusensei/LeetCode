using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int FindLengthOfShortestSubarray(int[] arr)
    {
        var right = arr.Length - 1;
        for (int i = arr.Length - 1; i > 0; i -= 1)
        {
            if (arr[i] >= arr[i - 1]) { right = i - 1; } else { break; }
        }
        var res = right;
        var left = 0;
        while (left < right && (left == 0 || arr[left - 1] <= arr[left]))
        {
            while (right < arr.Length && arr[left] > arr[right])
            {
                right += 1;
            }
            res = Math.Min(res, right - left - 1);
            left += 1;
        }
        return res;
    }
}
