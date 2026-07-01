using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindInMountainArray(int target, MountainArray mountainArr)
    {
        int n = mountainArr.Length();
        int left = 0;
        int right = n - 1;
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (mountainArr.Get(mid) > mountainArr.Get(1 + mid))
            {
                right = mid;
            }
            else { left = 1 + mid; }
        }
        int v = Find(0, left, (a, b) => a < b);
        if (v > -1) { return v; }
        else
        {
            v = Find(left, n - 1, (a, b) => a > b);
            return v > -1 ? v : -1;
        }

        int Find(int left, int right, Func<int, int, bool> f)
        {
            while (left < right)
            {
                int mid = left + (right - left) / 2;
                if (f(mountainArr.Get(mid), target))
                {
                    left = 1 + mid;
                }
                else { right = mid; }
            }
            return mountainArr.Get(left) == target ? left : -1;
        }
    }
}

// stub
public struct MountainArray
{
    public int Get(int _) => 0;
    public int Length() => 0;
}