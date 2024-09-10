using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = ListNode.Make([18, 6, 10, 3]);
        var b = sol.InsertGreatestCommonDivisors(a);
        var c = "[18,6,6,2,10,1,3]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = ListNode.Make([7]);
        var b = sol.InsertGreatestCommonDivisors(a);
        var c = "[7]";
        Assert.AreEqual(c, b.ToString());
    }
}