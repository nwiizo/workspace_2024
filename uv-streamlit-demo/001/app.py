import streamlit as st


def main():
    st.title("Hello World!")
    st.write("Welcome to the Streamlit version of the UV demo app!")

    # Add some interactive elements to showcase Streamlit features
    if st.button("Click me!"):
        st.balloons()

    st.sidebar.markdown(
        """
    ### About
    This is a simple Streamlit app demonstrating UV package management.
    """
    )


if __name__ == "__main__":
    main()
