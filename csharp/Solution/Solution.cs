using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool JudgePoint24(int[] cards)
    {
        const int n = 4;
        double[] nums = [.. cards];
        Array.Sort(nums);
        do
        {
            if (Dfs(nums)) { return true; }
        } while (NextPerm(nums));
        return false;

        static bool Dfs(IList<double> nums)
        {
            if (nums.Count == 1)
            {
                return double.Abs(nums[0] - 24) < double.Pow(10, -6);
            }
            for (int i = 0; i < nums.Count - 1; i++)
            {
                List<double> curr = [.. nums];
                curr.RemoveAt(1 + i);
                foreach (var item in Calc(nums[i], nums[1 + i]))
                {
                    curr[i] = item;
                    if (Dfs(curr)) { return true; }
                }
            }
            return false;
        }

        static IEnumerable<double> Calc(double a, double b)
        {
            yield return a + b;
            yield return a - b;
            yield return a * b;
            if (b != 0.0) { yield return a / b; }
        }

        static bool NextPerm(Span<double> nums)
        {
            int left = n - 2;
            bool found = false;
            for (; left >= 0; left -= 1)
            {
                if (nums[left] < nums[1 + left])
                {
                    found = true;
                    break;
                }
            }
            if (!found) { return false; }
            for (int right = n - 1; right >= 0; right -= 1)
            {
                if (left < right && nums[left] < nums[right])
                {
                    (nums[left], nums[right]) = (nums[right], nums[left]);
                    break;
                }
            }
            nums[(1 + left)..].Reverse();
            return true;
        }
    }
}