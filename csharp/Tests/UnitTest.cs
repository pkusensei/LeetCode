using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    // readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        var at = new MinStack();
        at.Push(-2);
        at.Push(0);
        at.Push(-3);
        Assert.AreEqual(-3, at.GetMin());
        at.Pop();
        Assert.AreEqual(0, at.Top());
        Assert.AreEqual(-2, at.GetMin());
    }

    [TestMethod]
    public void TestMethod2()
    {
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