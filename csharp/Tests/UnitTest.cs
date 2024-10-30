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
        Assert.IsTrue(sol.LemonadeChange([5, 5, 5, 10, 20]));
    }

    [TestMethod]
    public void TestMethod2()
    {
        Assert.IsFalse(sol.LemonadeChange([5, 5, 10, 10, 20]));
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