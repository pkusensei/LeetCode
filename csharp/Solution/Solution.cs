using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinEatingSpeed(int[] piles, int h)
    {
        int left = 1;
        int right = piles.Max();
        while (left < right)
        {
            int mid = left + (right - left) / 2;
            if (Eat(mid) > h) { left = 1 + mid; }
            else { right = mid; }
        }
        return left;

        int Eat(int mid) => piles.Sum(v => (v + mid - 1) / mid);
    }
}