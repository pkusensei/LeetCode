using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaximumSwap(int num)
    {
        var temp = num;
        var seq = new List<int>();
        while (temp > 0)
        {
            seq.Add(temp % 10);
            temp /= 10;
        }
        seq.Reverse();

        var digits = new int[10];
        foreach (var (d, i) in seq.Select((d, i) => (d, i)))
        {
            digits[d] = i; // last occurence of each digit
        }
        foreach (var (digit, left) in seq.Select((d, i) => (d, i)))
        {
            for (int larger = 9; larger > digit; larger -= 1)
            {
                int right = digits[larger];
                if (right > left) // Find a bigger digit to the right
                {
                    (seq[left], seq[right]) = (seq[right], seq[left]);
                    return seq.Aggregate(0, (acc, num) => acc * 10 + num);
                }
            }
        }
        return num;
    }
}
