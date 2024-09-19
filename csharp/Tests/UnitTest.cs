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
        var a1 = ListNode.Make([7, 2, 4, 3]);
        var a2 = ListNode.Make([5, 6, 4]);
        var b = sol.AddTwoNumbers(a1, a2);
        var c = "[7,8,0,7]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a1 = ListNode.Make([2, 4, 3]);
        var a2 = ListNode.Make([5, 6, 4]);
        var b = sol.AddTwoNumbers(a1, a2);
        var c = "[8,0,7]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a1 = ListNode.Make([0]);
        var a2 = ListNode.Make([0]);
        var b = sol.AddTwoNumbers(a1, a2);
        var c = "[0]";
        Assert.AreEqual(c, b.ToString());
    }

}