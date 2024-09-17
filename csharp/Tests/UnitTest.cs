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
        var b = sol.FindMaximumXOR([3, 10, 5, 25, 2, 8]);
        var c = 28;
        Assert.AreEqual(c, b);
    }

    [TestMethod]
    public void TestMethod2()
    {
        var b = sol.FindMaximumXOR([14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]);
        var c = 127;
        Assert.AreEqual(c, b);
    }
}