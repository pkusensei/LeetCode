using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumPairRemoval(int[] nums)
    {
        LinkedList<int> lst = new(nums);
        int res = 0;
        while (lst.Zip(lst.Skip(1)).Any(p => p.First > p.Second))
        {
            res += 1;
            int min_sum = int.MaxValue;
            var head = lst.First;
            var node = head;
            while (head.Next is not null)
            {
                int temp = head.Value + head.Next.Value;
                if (temp < min_sum)
                {
                    min_sum = temp;
                    node = head;
                }
                head = head.Next;
            }
            lst.Remove(node.Next);
            node.Value = min_sum;
        }
        return res;
    }
}
