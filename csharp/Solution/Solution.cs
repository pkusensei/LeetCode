using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int CountGoodTriplets(int[] arr, int a, int b, int c)
    {
        int res = 0;
        for (int i1 = 0; i1 < arr.Length; i1++)
        {
            for (int i2 = 1 + i1; i2 < arr.Length; i2++)
            {
                if (Math.Abs(arr[i1] - arr[i2]) > a) { continue; }
                for (int i3 = 1 + i2; i3 < arr.Length; i3++)
                {
                    if (Math.Abs(arr[i2] - arr[i3]) <= b && Math.Abs(arr[i3] - arr[i1]) <= c) { res += 1; }
                }
            }
        }
        return res;
    }
}
