using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void Merge(int[] nums1, int m, int[] nums2, int n)
    {
        int i1 = m - 1;
        int i2 = n - 1;
        for (int dst = m + n - 1; dst >= 0; dst -= 1)
        {
            if (i1 >= 0 && i2 >= 0)
            {
                if (nums1[i1] > nums2[i2])
                {
                    nums1[dst] = nums1[i1];
                    i1 -= 1;
                }
                else
                {
                    nums1[dst] = nums2[i2];
                    i2 -= 1;
                }
            }
            else if (i1 >= 0)
            {
                nums1[dst] = nums1[i1];
                i1 -= 1;
            }
            else
            {
                nums1[dst] = nums2[i2];
                i2 -= 1;
            }
        }
    }
}