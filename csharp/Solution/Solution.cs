using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string ComplexNumberMultiply(string num1, string num2)
    {
        (int a1, int b1) = Parse(num1);
        (int a2, int b2) = Parse(num2);
        int a = a1 * a2 - b1 * b2;
        int b = a1 * b2 + a2 * b1;
        return $"{a}+{b}i";

        // a + bi
        static (int a, int b) Parse(string s)
        {
            Span<int> vals = stackalloc int[2];
            int i = 0;
            foreach (var item in s.Split('+'))
            {
                if (i == 1) { vals[i] = int.Parse(item.AsSpan()[..(item.Length - 1)]); }
                else { vals[i] = int.Parse(item); }
                i += 1;
            }
            return (vals[0], vals[1]);
        }
    }
}
