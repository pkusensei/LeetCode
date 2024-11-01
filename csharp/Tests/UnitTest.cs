using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var a = ListNode.Make([1, 2, 3, 4, 5]);
        var b = sol.MiddleNode(a);
        var c = "[3,4,5]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = ListNode.Make([1, 2, 3, 4, 5, 6]);
        var b = sol.MiddleNode(a);
        var c = "[4,5,6]";
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}