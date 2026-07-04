using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] CorpFlightBookings(int[][] bookings, int n)
    {
        int[] diff = new int[n];
        foreach (var item in bookings)
        {
            diff[item[0] - 1] += item[2];
            if (item[1] < n) { diff[item[1]] -= item[2]; }
        }
        for (int i = 0; i < n - 1; i++)
        {
            diff[1 + i] += diff[i];
        }
        return diff;
    }
}
