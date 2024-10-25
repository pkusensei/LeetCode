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
        var a = ListNode.Make([0, 1, 2, 3]);
        var b = sol.NumComponents(a, [0, 1, 3]);
        var c = 2;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = ListNode.Make([0, 1, 2, 3, 4]);
        var b = sol.NumComponents(a, [0, 3, 1, 4]);
        var c = 2;
        Assert.AreEqual(c, b);
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