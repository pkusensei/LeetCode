using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("12", 2)]
    [DataRow("2611055971756562", 4)]
    [DataRow("1", 1)]
    public void TestMethod1(string s, int exp)
    {
        Assert.AreEqual(exp, sol.NumDecodings(s));
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